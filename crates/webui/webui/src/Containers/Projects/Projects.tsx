import styled from "@emotion/styled";
import { FC } from "react";
import AllProjects from "../../Components/AllProjects";
import Navbar from "../../Components/Navbar";
import Header from "./Header";

const Wrapper = styled.div`
  height: calc(100vh - 60px);
  overflow-y: auto;
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;
  max-width: 56rem;
  margin: 0 auto;
  margin-top: 2rem;
  margin-bottom: 3rem;
`;

const Projects: FC = () => {
  return (
    <>
      <Navbar />
      <Wrapper>
        <Container>
          <Header />
          <AllProjects />
        </Container>
      </Wrapper>
    </>
  );
};

export default Projects;
