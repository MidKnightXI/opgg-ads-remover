import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";

import "./App.css";
import opgg from "./assets/opgg.svg";


function App() {
  const [status, setStatus] = createSignal("");

  async function patch() {
    const res = await invoke("patch")
    setStatus(res);
  }

  return (
    <div class="container">
      <a href="https://github.com/MidKnightXI/opgg-ads-remover" target="_blank">
        <img src={opgg} class="logo opgg" alt="OPGG logo" />
      </a>
      <p>Click on the logo to get redirected to the github page.</p>
      <br/>
      <button type="button" onClick={() => patch()}>
        Patch
      </button>
      <p>{status}</p>
      <p id="copyright">{`2022 - ${new Date().getFullYear()} OPGG Patcher - All Rights Reserved.`}</p>
    </div>
  );
}

export default App;
