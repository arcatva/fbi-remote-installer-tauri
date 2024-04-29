import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import Table from "./components/Table";
import Banner from "./components/Banner.jsx";

function App() {
	const [ip, setIp] = useState("");
	const [files, setFiles] = useState([]);
	const [bannerStatus, setBannerStatus] = useState("off");
	const [bannerMessage, setBannerMessage] = useState("");


	function setBanner(status, message) {
		setBannerStatus(status);
		setBannerMessage(message);
		setTimeout(
			() => {
				setBannerStatus("off");
				setBannerMessage("");
			}, 10000
		)
	}

	async function list_files() {
		let result = await invoke("list_files");
		console.log(result);
		setFiles(result);
	}

	async function send_file(filePath) {
		invoke("send_file", {addr: ip, filePath: filePath})
			.then((result) => {
				console.log(result);
				setBanner("success", result);
			})
			.catch((err) => {
				setBanner("danger", err);
				console.log(err);
			});
	}

	useEffect(() => {
		list_files();
	}, []);
	return (
		<div className="bg-gray-700 w-screen h-screen p-10">
			<Banner level={bannerStatus} message={bannerMessage}/>

			<div className="text-center font-sans text-4xl text-white p-10">
				FBI Remote Installer Tauri
			</div>

			<form
				className="flex justify-center items-center space-x-4"
				onSubmit={(e) => {
					e.preventDefault();

				}}
			>
				<input
					className="rounded-2xl py-2 px-3 bg-slate-900 text-white text-sm"
					onChange={(e) => setIp(e.currentTarget.value)}
					placeholder="Enter 3DS IP:Port"

				/>

			</form>
			<div className="flex-row justify-center items-center bg-white m-10 space-y-2 rounded-2xl">
				<Table files={files} sendfile={send_file}/>
			</div>
		</div>
	);
}

export default App;
