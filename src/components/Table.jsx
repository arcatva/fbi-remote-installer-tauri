import { useState } from "react";

function TableHead() {
  return (
    <thead class="text-center ">
      <tr>
        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
          Name
        </th>
        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
          Size
        </th>
        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
          Last Modified
        </th>
        <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
          Action
        </th>
      </tr>
    </thead>
  );
}

function Table({ files }) {
  const [page, setPage] = useState(1);
  const filesPerPage = 5;
  const pageCount = Math.ceil(files.length / filesPerPage);
  return (
    <>
      <table class="min-w-full divide-y-2 divide-gray-200 bg-white text-sm rounded-2xl ">
        <TableHead />
        <tbody class="divide-y divide-gray-200 text-center ">
          {files.map((element, index) => {
            if (index < page * 5 && index >= (page - 1) * 5) {
              return (
                <tr key={index}>
                  <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
                    {element.file_size == 0
                      ? element.file_name + "/"
                      : element.file_name}
                  </td>
                  <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                    {element.file_size == 0
                      ? ""
                      : element.file_size / 1000 + " KB"}
                  </td>
                  <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                    {element.modified}
                  </td>
                  <td class="whitespace-nowrap px-4 py-2">
                    {element.file_size == 0 ? (
                      ""
                    ) : (
                      <div class="rounded-2xl inline-block  px-4 py-2 text-xs font-medium text-white bg-blue-300 hover:bg-blue-400 ">
                        Install
                      </div>
                    )}
                  </td>
                </tr>
              );
            }
          })}
        </tbody>
      </table>
      <div class="border-t border-gray-200 px-4 py-2">
        <ol class="flex justify-end gap-1 text-xs font-medium">
          <li>
            <div
              onClick={() => {
                if (page > 1) {
                  setPage(page - 1);
                }
              }}
              class="select-none cursor-pointer hover:ring-2 inline-flex size-8 items-center justify-center rounded-2xl border border-gray-100 bg-white text-gray-900 rtl:rotate-180"
            >
              <span class="sr-only">Prev Page</span>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-3 w-3"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fill-rule="evenodd"
                  d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                  clip-rule="evenodd"
                />
              </svg>
            </div>
          </li>
          {Array.from({ length: pageCount }, (_, i) => i + 1).map((index) =>
            index == page ? (
              <li key={index}>
                <div className="select-none  hover:ring-2 block size-8 rounded-2xl border border-gray-100 bg-blue-300 text-center leading-8 text-gray-100">
                  {index}
                </div>
              </li>
            ) : (
              <li key={index}>
                <div
                  onClick={() => {
                    setPage(index);
                  }}
                  className="select-none cursor-pointer hover:ring-2 block size-8 rounded-2xl border border-gray-100 bg-white text-center leading-8 text-gray-900"
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
              class="select-none cursor-pointer hover:ring-2 inline-flex size-8 items-center justify-center rounded-2xl border border-gray-100 bg-white text-gray-900 rtl:rotate-180"
            >
              <span class="sr-only">Next Page</span>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-3 w-3"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fill-rule="evenodd"
                  d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                  clip-rule="evenodd"
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
