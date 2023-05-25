import { FC } from "react";
import Navbar from "../../Components/Navbar";
import styled from "@emotion/styled";
import Header from "./Header";
import Background from "../../Components/ServicesGraph/Background";
import { Input } from "baseui/input";
import Templates from "./Templates";

const Container = styled.div`
  display: flex;
  flex-direction: column;
  max-width: 56rem;
  margin: 0 auto;
  margin-top: 2rem;
  align-items: center;
`;

const Card = styled.div`
  width: 500px;
  min-height: 200px;
  max-height: 350px;
  background-color: #fff;
  border-radius: 10px;
  z-index: 2;
`;

const CardContent = styled.div`
  border-top: 1px solid #eaeaea;
  min-height: 200px;
  max-height: 300px;
  overflow-y: auto;
  overflow-x: hidden;
`;

const NewProject: FC = () => {
  return (
    <div>
      <Navbar />
      <div
        style={{
          position: "absolute",
          width: "calc(100% - 15px)",
          height: "calc(100% - 60px)",
          backgroundColor: "#a0cfe812",
        }}
      >
        <Background />
      </div>
      <Container>
        <Header />
        <Card>
          <Input
            placeholder="Create a new project"
            overrides={{
              Root: {
                style: () => ({
                  border: "none",
                  borderRadius: "0px",
                }),
              },
              InputContainer: {
                style: () => ({
                  backgroundColor: "#fff",
                }),
              },
            }}
          />
          <CardContent>
            <Templates />
          </CardContent>
        </Card>
      </Container>
    </div>
  );
};

export default NewProject;
