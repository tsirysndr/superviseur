import Header from "./Header";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

const HeaderWithData: FC = () => {
  const navigate = useNavigate();
  return (
    <>
      <Header onCreateProject={() => navigate("/new")} />
    </>
  );
};

export default HeaderWithData;
