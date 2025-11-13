import Image from "./image";
import File from "./file";
import { convertFileSrc } from "@tauri-apps/api/core";

function GalleryFile({ file }) {
  const fileExtension = file.name.split(".")[1];
  const isImage =
    fileExtension === "jpeg" ||
    fileExtension === "png" ||
    fileExtension === "jpg";

  console.log(file);

  return isImage ? (
    <Image src={convertFileSrc(file.path)} name={file.name} />
  ) : (
    <File file={file} />
  );
}

export default GalleryFile;
