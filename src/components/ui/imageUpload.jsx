import { Upload } from "lucide-react";
import { Label } from "./label";
import { open } from "@tauri-apps/plugin-dialog";
import { basename } from "@tauri-apps/api/path";

function ImageUpload({ setVisit, setPaths }) {
  return (
    <div className="">
      <Label className="text-md text-text-secondary mb-1">
        Adjuntar imagenes
      </Label>

      <label
        onClick={async (e) => {
          const files = await open({
            multiple: true,
            directory: false,
          });
          if (!files) return;

          const docs = await Promise.all(
            files.map(async (x) => ({
              name: await basename(x),
              path: x,
            }))
          );

          setVisit((prev) => ({
            ...prev,
            docs,
          }));
        }}
        className=""
        htmlFor="upload"
      >
        <div className="flex flex-col items-center bg-[#f9fafb] rounded-md p-20 hover:opacity-70 cursor-pointer">
          <Upload className="text-text-secondary size-10" />
          <p className="text-text-secondary font-bold">Click para subir</p>
          <p className="text-text-secondary text-sm">PNG o JPG</p>
        </div>
      </label>
    </div>
  );
}

export default ImageUpload;
