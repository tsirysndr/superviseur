import { FC, useState } from "react";
import { Input, SIZE } from "baseui/input";
import styled from "@emotion/styled";
import { SearchOutline } from "@styled-icons/evaicons-outline/SearchOutline";

const Container = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  background-color: #fff;
  height: 40px;
`;

const Logo = styled.div`
  color: #31fff3;
  font-weight: bold;
  width: 130px;
  text-align: center;
`;

const Settings = styled.div`
  width: 200px;
`;

interface NavbarProps {}

const Navbar: FC<NavbarProps> = () => {
  const [value, setValue] = useState("");
  return (
    <Container>
      <Logo>Superviseur</Logo>
      <Input
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search for a service ..."
        clearOnEscape
        size={SIZE.default}
        startEnhancer={() => <SearchOutline size={20} color="#b3b3b3" />}
        overrides={{
          Root: {
            style: {
              width: "400px",
              height: "35px",
              borderWidth: "1.5px",
              borderRadius: "18px",
              backgroundColor: "#fff",
            },
          },
          InputContainer: {
            style: {
              backgroundColor: "#fff",
            },
          },
          StartEnhancer: {
            style: {
              paddingLeft: "0px",
              backgroundColor: "#fff",
            },
          },
        }}
      />
      <Settings></Settings>
    </Container>
  );
};

export default Navbar;
