import { BrowserRouter, Routes, Route } from "react-router-dom";
import Dashboard from "./Containers/Dashboard";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
