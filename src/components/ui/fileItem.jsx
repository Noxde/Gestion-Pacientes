import { convertFileSrc } from "@tauri-apps/api/core";
import { Button } from "../shadcn/button";
import { File, X } from "lucide-react";

function FileItem({ file, onClick }) {
  const fileExtension = file.name.split(".")[1];
  const isImage =
    fileExtension === "jpeg" ||
    fileExtension === "png" ||
    fileExtension === "jpg";

  return (
    <div className="bg-[#f9fafb] p-2 rounded-md flex items-center justify-between">
      <div>
        {isImage ? (
          <img
            className="inline-block object-fill size-15 mr-2 rounded-md"
            src={convertFileSrc(file.path)}
            alt=""
          />
        ) : (
          <File className="inline-block mx-5 text-text-secondary" />
        )}
        <span className="text-text-secondary">{file.name}</span>
      </div>
      <Button
        onClick={() => onClick(file)}
        variant="ghost"
        className="cursor-pointer"
      >
        <X />
      </Button>
    </div>
  );
}

export default FileItem;
