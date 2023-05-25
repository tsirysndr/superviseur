import { BrowserRouter, Routes, Route } from "react-router-dom";
import Dashboard from "./Containers/Dashboard";
import NewProject from "./Containers/NewProject";
import Projects from "./Containers/Projects";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Projects />} />
        <Route path="/new" element={<NewProject />} />
        <Route path="/projects/:projectId" element={<Dashboard />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
