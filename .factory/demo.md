# Demo sandbox

## Browser demo

Open `https://config-rationale-guard.sociobot.in/demo` or select **Try it with
sample data** on the landing page. The page immediately contains a realistic
agent configuration with shell permissions, a package-mirror network setting,
and two adjacent decision records.

The visible banner reads **Demo — sample data, nothing is saved**. Demo input is
stored only under `demo:config-rationale-guard:checker` in browser local
storage. It never reads from or writes to a non-demo key. **Reset demo** restores
the shipped sample and clears that key. **Start for real** clears it before
returning to the landing page.

## CLI demo

Run `crg demo` after installing the CLI. It creates a new temporary directory
with `agent.json`, `agent.json.rationale.json`, and `agent.schema.json`, then
runs a real `crg check` against them. `crg demo --output ./new-folder` uses a
new or empty folder.

The browser and CLI samples are bundled with the repository. No account,
credential, network request, or real project data is required.
