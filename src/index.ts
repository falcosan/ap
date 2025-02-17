import "./index.css";
import init from "../rust/pkg";

init().then(({ app }) => app());
