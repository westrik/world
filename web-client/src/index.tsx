import * as React from "react";
import { render } from "react-dom";

const App: React.SFC = () => <h1>Hello world</h1>;

render(<App />, document.getElementById("root"));

// @ts-ignore
if (module.hot) {
  // @ts-ignore
  module.hot.accept();
}
