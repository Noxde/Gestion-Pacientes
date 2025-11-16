import GalleryFile from "./galleryFile";

function FilesGallery({ files }) {
  return (
    <div className="grid grid-cols-5 auto-rows-[150px] gap-2">
      {!files.length ? (
        <div>No hay documentos</div>
      ) : (
        <>
          {files.map((x) => (
            <GalleryFile file={x} />
          ))}
        </>
      )}
    </div>
  );
}

export default FilesGallery;
