import { Upload } from "lucide-react";
import { Label } from "./label";
import { open } from "@tauri-apps/plugin-dialog";
import { basename } from "@tauri-apps/api/path";
import FileItem from "./fileItem";

function ImageUpload({ setFiles, files }) {
  function handleRemove(e) {
    setFiles((prev) => prev.filter((x) => x.name !== e.name));
  }

  return (
    <div>
      <Label className="text-md text-text-secondary mb-1">
        Adjuntar imagenes
      </Label>

      <label
        onClick={async (e) => {
          const selectedFiles = await open({
            multiple: true,
            directory: false,
            filters: [
              {
                name: "Imagenes (png, jpg, jpeg)",
                extensions: ["png", "jpg", "jpeg"],
              },
              {
                name: "Videos (mp4)",
                extensions: ["mp4"],
              },
              {
                name: "Documentos (pdf, docx)",
                extensions: ["pdf", "docx"],
              },
            ],
          });
          if (!selectedFiles) return;
          setIsLoading(true);

          const docs = await Promise.all(
            selectedFiles.map(async (x) => ({
              name: await basename(x),
              path: x,
            }))
          );

          setFiles((prev) => prev.concat(docs));
        }}
        className=""
        htmlFor="upload"
      >
        <div
          className={`flex flex-col items-center justify-center bg-[#f9fafb] rounded-md hover:opacity-70 cursor-pointer ${
            !files.length ? "p-20" : "p-5"
          }`}
        >
          <Upload className="text-text-secondary size-10" />
          <p className="text-text-secondary font-bold">Click para subir</p>
          {!files.length && (
            <p className="text-text-secondary text-sm">
              Archivos permitidos (.jpg/jpeg, .png, .mp4, .docx, .pdf)
            </p>
          )}
        </div>
      </label>
      {/* Files list */}
      {files.length ? (
        <div className="flex flex-col gap-2 mt-2 max-h-[20dvh] overflow-y-auto">
          {files.map((x) => (
            <FileItem file={x} onClick={handleRemove} />
          ))}
        </div>
      ) : null}
    </div>
  );
}

export default ImageUpload;
