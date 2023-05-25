import styled from "@emotion/styled";
import { FC } from "react";

const Wrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1;
  margin-bottom: 4rem;
`;

const Title = styled.h2`
  font-size: 28px;
  z-index: 1;
  text-align: center;
  margin-bottom: 15px;
`;

const SubTitle = styled.div`
  font-size: 17px;
  z-index: 1;
  color: #302f2f;
  text-align: center;
`;

const Purple = styled.span`
  color: #630be2;
  cursor: pointer;
`;

const Header: FC = () => {
  return (
    <>
      <Wrapper>
        <div>
          <Title>New Project</Title>
          <SubTitle>
            Start a new project by selecting a <Purple>template</Purple> or{" "}
            <Purple>importing</Purple> an existing project.
          </SubTitle>
        </div>
      </Wrapper>
    </>
  );
};

export default Header;
