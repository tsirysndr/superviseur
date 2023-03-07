import { BrowserRouter, Routes, Route } from "react-router-dom";
import Dashboard from "./Containers/Dashboard";
import { services } from "./Containers/Dashboard/Dashboard.stories";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard services={services} />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
