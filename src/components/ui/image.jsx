function Image({ src, name }) {
  return (
    <div className="rounded-2xl overflow-hidden relative">
      {/* Name */}
      <div className="absolute z-50 bg-linear-to-t from-[#000000e8] to-transparent p-2 h-full w-full flex items-end">
        <span className="text-white font-semibold">{name}</span>
      </div>
      <img className="w-full h-full inline-block object-fill" src={src} />
    </div>
  );
}

export default Image;
