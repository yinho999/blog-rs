import "./App.css";
import AppRouter from "./router";
import { initialState, reducer } from "./reducer";
import { useReducer } from "react";
import { AppContext } from "./context/AppContext";
import "@mui/material/styles";
// import Header from "./components/Header";
const App = () => {
  const [state, dispatch] = useReducer(reducer, initialState);

  return (
    <>
      {/*<Header />*/}
      <AppContext.Provider value={{ state, dispatch }}>
        <AppRouter />
      </AppContext.Provider>
    </>
  );
};

export default App;
