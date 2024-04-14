import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import Table from "./components/Table";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [files, setFiles] = useState([]);

  async function list_files() {
    let result = await invoke("list_files");
    setFiles(result);
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }
  useEffect(() => {
    list_files();
  }, []);
  return (
    <div className="bg-slate-800 w-full h-full p-10">
      <div className="text-center font-sans text-4xl text-white pt-10">
        FBI Remote Installer Tauri
      </div>
      <div className="flex justify-center items-center scale-50">
        <img src="/tauri.svg" alt="Tauri logo" />
      </div>

      <form
        className="flex justify-center items-center space-x-2"
        onSubmit={(e) => {
          e.preventDefault();
          list_files();
        }}
      >
        <input
          className="rounded-md p-2 bg-slate-900 text-white"
          id="tcp-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button className="font-sans text-white" type="submit">
          Greet
        </button>
      </form>

      <p>{greetMsg}</p>

      <div className="flex-row justify-center items-center bg-white m-10 space-y-2 rounded-md">
        {/*         <div className="flex">
          <div>FileName</div>
        </div>
        {files.map((element, index) => (
          <div key={index}>{element.file_name}</div>
        ))} */}

        <Table files={files} />
      </div>
    </div>
  );
}

export default App;
