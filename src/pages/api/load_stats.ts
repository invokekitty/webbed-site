import type { APIContext, APIRoute } from "astro";

const ALL_MODULES = <const>["cpu", "memory", "gpu", "downlink", "uplink"]
type Module = typeof ALL_MODULES[number];
type DataPoint = { percentage: number, value?: number, string?: string }

const COLORS: Record<Module, string> = {
  cpu: "#89b4fa",
  memory: "#a6e3a1",
  gpu: "#f38ba8",
  downlink: "#fab387",
  uplink: "#89dceb"
}

export async function GET({ url, request }: APIContext): Promise<Response> {
  function isModule(str: string): str is Module {
    return ALL_MODULES.some((lit) => str === lit);
  }

  const modules = url.searchParams.getAll("modules").filter(isModule);
  const size = Number.parseInt(url.searchParams.get("count") ?? "40");

  let intervalId: number | null = null
  const encoder = new TextEncoder();

  function end() {
    if (intervalId) clearInterval(intervalId)
    console.log("Statistic connection finished")
  }

  const stream = new ReadableStream({
    start(controller) {
      console.log("Statistic connection opened");
      intervalId = runStream(size, modules, (data: string) => {
        if (controller.desiredSize === null || controller.desiredSize < 0) {
          end();
          controller.close();
          return;
        }
        controller.enqueue(encoder.encode(data));
      });
      setTimeout(controller.close, 300000);

      return end;
    },

    cancel() {
      end()
    }
  })


  return new Response(stream, {
    status: 200,
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Transfer-Encoding': 'chunked',
      'Connection': 'keep-alive',
      'Cache-Control': 'no-cache, no-transform',
      'Cloudflare-CDN-Cache-Control': 'no-cache'
    }
  });
}


function runStream(points: number, enabled: Module[], send: (data: string) => void): number {
  function* range(from: number, to: number): Generator<number, void, unknown> {
    while(from < to) yield from++
  }

  send(`
    <html>
      <head>
        <title>graph</title>
        <style>
          * { box-sizing: border-box; color: #cdd6f4; }
          body { background: none transparent; width: 100vw; height: 100vh; margin:0; }
          .gc { display: block; width: 100%; height: 100%; }
          .dt { position: absolute; width: 100%; height: 16px; transform: translate(0, 50%); }
          .dtc { position: absolute; bottom: 0; width: ${(1 / points) * 100}%; height: 100%; transform: translate(-50%, 0); z-index: 20; }
          .dtc:hover:before { content:' '; position: absolute; bottom: 0; height: 100%; width: 2px; background-color: #a6adc8aa; left: 50%; transform: translate(-50%, 0); mask: linear-gradient(to top, black, transparent); }
          .dt:after { display:none; }
          .dtc:hover>.dt:after {
              position: absolute;left: 50%;top: -.0.5rem;transform: translate(-50%, -100%);border-radius: 2px;background-color: #1e1e2e;
              color: #cdd6f4;font-size: small;font-family: monospace;padding: 0.35rem;font-weight:
              bold;text-wrap: nowrap;z-index: 299; display: block;
          }
          .g { position: fixed; left: 0; top: 0; width: 100%; height: 100%; padding-left: 8px; padding-bottom: 8px; }
        </style>
      </head>
      <body>
        <div class='g'>
          <template>
            <slot width="100%" height="100%" name='gc'></slot>
          </template>
          ${
            range(0, points)
              .map(i => `
                <span class='dtc' style='left:${i * (100 / points)}%'>${enabled.map(m => `<span class='dt' id='T${m}${i}'></span>`).join("")}</span>
                `)
              .toArray().join("")
          }
        </div>
    `);

  let count = 0;

  function action() {
    count++;

    queryData(points).then(all => {
      const modules = all.filter(([module]) => enabled.includes(module))
      const lines = modules
        .map(([ module, data ]) =>
          `<polyline
            points="${data.toReversed().map(({ percentage }, index) => `${index * (200 / points)} ${100 - percentage}`).join(', ')}"
            style="fill:none;stroke:${COLORS[module]};stroke-width:0.5"
          />`
        )
        .join("")

      const tooltips = modules.flatMap(([module, data]) => data.toReversed().map(({ percentage, string }, index) =>
        `#T${module}${index}{bottom:${percentage}%;}#T${module}${index}:after{content:'${string}';}`
      )).join("")

      send(`
          <span slot='gc' class='gc' id='s${count}'><svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 200 100">${lines}</svg></span>
          <style>#s${count - 1}{display: none;}${tooltips}</style>
        `)
    }).catch(err => console.log(err));
  };

  action();
  return setInterval(action, 1000);
}

const ENDPOINT = "https://ryusei.kisse.su/sysinfo_back"

async function queryData(points: number): Promise<[Module, readonly DataPoint[]][]> {
  let response = await fetch(`${ENDPOINT}_${points}`);
  let data = await response.json() as any[];

  function networkify(value: number): string {
    const kib = ((value * 8) / 1000)
    if (kib < 1000) return `${kib.toFixed(2)} kbit/s`;
    if (kib < 1000000) return `${(kib / 1000).toFixed(2)} mbit/s`
    return `${(kib / 10000000).toFixed(2)} gbit/s`
  }

  let cpu = data.map<DataPoint>(point => ({
    percentage: point.cpu_usage,
    string: `${point.cpu_usage.toFixed(2)}% @ ${(point.cpu_frequency / 1000).toFixed(2)} GHz`
  }));
  let memory = data.map<DataPoint>(point => ({
    percentage: ((point.memory_usage) / (point.memory_size)) * 100,
    value: point.memory_usage,
    string: `${(point.memory_usage / 1073741824).toFixed(2)} GiB / ${(point.memory_size / 1073741824).toFixed(2)} GiB`
  }));
  let gpu = data.map<DataPoint>(point => ({
    percentage: point.gpu_usage,
    string: `${point.gpu_usage.toFixed(2)}%`
  }));
  let downlink = data.map<DataPoint>(point => ({
    percentage: (point.downlink / point.downlink_max) * 100,
    string: networkify(point.downlink)
  }));
  let uplink = data.map<DataPoint>(point => ({
    percentage: (point.uplink / point.uplink_max) * 100,
    string: networkify(point.uplink)
  }))

  return [
    ["cpu", cpu],
    ["memory", memory],
    ["gpu", gpu],
    ["downlink", downlink],
    ["uplink", uplink]
  ];
}
