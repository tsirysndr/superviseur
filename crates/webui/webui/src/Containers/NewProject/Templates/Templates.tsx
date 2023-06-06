import { FC } from "react";
import { Stack } from "@styled-icons/octicons";
import styled from "@emotion/styled";

const Action = styled.div`
  height: 50px;
  width: 100%;
  display: flex;
  align-items: center;
  padding-left: 20px;
  cursor: pointer;
  &:hover {
    color: #630be2;
    background-color: #fbfbfb;
  }
`;

const Logo = styled.img`
  height: 18px;
  margin-right: 15px;
`;

const Tag = styled.span`
  background-color: #650be214;
  color: #630be2;
  border-radius: 4px;
  font-size: 12px;
  padding: 2px 8px;
  margin-left: 12px;
`;

export type TemplatesProps = {
  templates: any[];
  onCreateNewProject: (template: any) => void;
};

const Templates: FC<TemplatesProps> = ({ templates, onCreateNewProject }) => {
  return (
    <>
      {templates.map((template) => (
        <Action key={template.id} onClick={() => onCreateNewProject(template)}>
          {template.icon && <Logo src={template.icon} />}
          {!template.icon && (
            <Stack color="#ff0a80" size={20} style={{ marginRight: 15 }} />
          )}
          <div>{template.name}</div>
          {template.tags.map((tag: string) => (
            <Tag key={tag}>{tag}</Tag>
          ))}
        </Action>
      ))}
    </>
  );
};

export default Templates;
