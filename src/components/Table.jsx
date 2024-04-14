function Table({ files }) {
  return (
    <div class="">
      <table class="min-w-full divide-y-2 divide-gray-200 bg-white text-sm rounded-md">
        <thead class="text-left">
          <tr>
            <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
              Name
            </th>
            <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
              Size
            </th>
            <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">
              Action
            </th>
          </tr>
        </thead>

        <tbody class="divide-y divide-gray-200">
          {files.map((element, index) => {
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
                <td class="whitespace-nowrap px-4 py-2">
                  <a
                    href="#"
                    class="inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
                  >
                    Install
                  </a>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

export default Table;
