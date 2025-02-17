import "./index.css";
import init from "../wasm/pkg";

init().then(({ app }) => app());
