import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Table from "./components/Table";
import Button from "./components/buttons/Button";

function App() {
  const [ip, setIp] = useState("");
  const [files, setFiles] = useState([]);

  async function list_files() {
    let result = await invoke("list_files");
    console.log(result);
    setFiles(result);
  }
  async function connect_tcp(ip) {
    invoke("connect_tcp", { addr: ip })
      .then((result) => console.log(result))
      .catch((err) => console.log(err));
  }

  async function sendfile_tcp(filePath) {
    invoke("sendfile_tcp", { addr: ip, filePath: filePath })
      .then((result) => console.log(result))
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    list_files();
  }, []);
  return (
    <div className="bg-gray-700 w-screen h-screen p-10">
      <div className="text-center font-sans text-4xl text-white p-10">
        FBI Remote Installer Tauri
      </div>

      <form
        className="flex justify-center items-center space-x-4"
        onSubmit={(e) => {
          e.preventDefault();
          list_files();
        }}
      >
        <input
          className="rounded-2xl py-2 px-3 bg-slate-900 text-white text-sm"
          onChange={(e) => setIp(e.currentTarget.value)}
          placeholder="Enter 3DS IP:Port"
        />
        <Button text={"Connect"} action={connect_tcp} param={ip} />
      </form>
      <div className="flex-row justify-center items-center bg-white m-10 space-y-2 rounded-2xl">
        <Table files={files} sendfile_tcp={sendfile_tcp} />
      </div>
    </div>
  );
}

export default App;
