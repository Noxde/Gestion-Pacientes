import { File as FileIcon } from "lucide-react";

function File({ file }) {
  return (
    <div className="h-full rounded-2xl p-2 border bg-white flex flex-col justify-center items-center gap-2">
      <FileIcon className="text-text-secondary" />
      <span
        className="text-text-secondary truncate w-full text-center"
        title={file.name}
      >
        {file.name}
      </span>
    </div>
  );
}

export default File;
