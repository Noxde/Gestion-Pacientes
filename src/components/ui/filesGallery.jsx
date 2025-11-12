import { convertFileSrc } from "@tauri-apps/api/core";

//TODO: Acomodar imagenes y mostrar archivos de otra forma

function FilesGallery({ files }) {
  return (
    <div>
      {!files.length ? (
        <div>No hay documentos</div>
      ) : (
        <div>
          {files.map((x) => (
            <img src={convertFileSrc(x.path)} alt="" />
          ))}
        </div>
      )}
    </div>
  );
}

export default FilesGallery;
