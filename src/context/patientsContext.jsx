import { createContext, useState } from "react";

export const PatientsContext = createContext();

export const PatientsContextProvider = function ({ children }) {
  const [selected, setSelected] = useState(null);
  const [patients, setPatients] = useState([]);

  return (
    <PatientsContext.Provider
      value={{
        selected,
        setSelected,
        patients,
        setPatients,
      }}
    >
      {children}
    </PatientsContext.Provider>
  );
};
