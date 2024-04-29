import {useState} from "react";
import Button from "./buttons/Button";
import formatBytes from "../utils/formatBytes.js";

function TableHead() {
	return (
		<thead>
		<tr>
			<th className="whitespace-nowrap px-4 py-2 font-medium text-on-primary">
				Name
			</th>
			<th className="whitespace-nowrap px-4 py-2 font-medium text-on-primary">
				Size
			</th>
			<th className="whitespace-nowrap px-4 py-2 font-medium text-on-primary">
				Last Modified
			</th>
			<th className="whitespace-nowrap px-4 py-2 font-medium text-on-primary">
				Action
			</th>
		</tr>
		</thead>
	);
}

function Table({files, sendfile}) {
	const [page, setPage] = useState(1);
	const filesPerPage = 5;
	const pageCount = Math.ceil(files.length / filesPerPage);
	return (
		<>
			<table className={"bg-primary text-sm rounded-2xl table-fixed w-full"}>
				<TableHead/>
				<tbody className={"text-center"}>
				{files.map((element, index) => {
					if (index < page * 5 && index >= (page - 1) * 5) {
						return (
							<tr key={index} className="text-xs font-sans">
								<td className="whitespace-nowrap px-4 py-2  font-medium overflow-hidden overflow-ellipsis">
									{element.file_name}
								</td>
								<td className="whitespace-nowrap px-4 py-2 ">
									{element.file_size === 0
										? ""
										: formatBytes(element.file_size)}
								</td>
								<td className="whitespace-nowrap px-4 py-2">
									{element.modified}
								</td>
								<td className="whitespace-nowrap px-4 py-2">
									{element.file_size === 0 ? (
										""
									) : (
										<Button
											text={"Install"}
											action={sendfile}
											param={element.file_name}
										/>
									)}
								</td>
							</tr>
						);
					}
				})}
				</tbody>
			</table>
			<div className="px-4 py-2">
				<ol className="flex justify-end gap-1 text-xs font-medium">
					<li>
						<div
							onClick={() => {
								if (page > 1) {
									setPage(page - 1);
								}
							}}
							className={page === 1 ? "select-none inline-flex size-8 items-center justify-center rounded-2xl" : "select-none cursor-pointer inline-flex size-8 items-center justify-center rounded-2xl interactive-bg-primary-container"}
						>
							<span className="sr-only">Prev Page</span>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								className="h-3 w-3"
								viewBox="0 0 20 20"
								fill="currentColor"
							>
								<path
									fillRule="evenodd"
									d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
									clipRule="evenodd"
								/>
							</svg>
						</div>
					</li>
					{Array.from({length: pageCount}, (_, i) => i + 1).map((index) =>
						index === page ? (
							<li key={index}>
								<div
									className="select-none bg-primary-container block size-8 rounded-2xl  text-center leading-8 ">
									{index}
								</div>
							</li>
						) : (
							<li key={index}>
								<div
									onClick={() => {
										setPage(index);
									}}
									className="select-none cursor-pointer  block size-8 rounded-2xl  text-center leading-8"
								>
									{index}
								</div>
							</li>
						)
					)}

					<li>
						<div
							onClick={() => {
								if (page < pageCount) {
									setPage(page + 1);
								}
							}}
							className={page === pageCount ? "select-none inline-flex size-8 items-center justify-center rounded-2xl" : "select-none cursor-pointer inline-flex size-8 items-center justify-center rounded-2xl interactive-bg-primary-container"}
						>
							<span className="sr-only">Next Page</span>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								className="h-3 w-3"
								viewBox="0 0 20 20"
								fill="currentColor"
							>
								<path
									fillRule="evenodd"
									d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
									clipRule="evenodd"
								/>
							</svg>
						</div>
					</li>
				</ol>
			</div>
		</>
	);
}

export default Table;
