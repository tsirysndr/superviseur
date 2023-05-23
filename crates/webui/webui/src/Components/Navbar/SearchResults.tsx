import styled from "@emotion/styled";
import { uniqueId, startCase } from "lodash";
import { FC, useState } from "react";
import { Service } from "../../Hooks/GraphQL";
import { ListItem, ListItemLabel } from "baseui/list";
import { Terminal } from "@styled-icons/fa-solid/Terminal";

const Container = styled.div`
  min-height: 50px;
  width: 400px;
`;

const Status = styled.div`
  font-size: 12.5px;
  color: #5b5b5bac;
  font-family: RockfordSansMedium;
`;

export interface SearchResultsProps {
  results: Service[];
  onSelect: (id: string) => void;
}

const SearchResults: FC<SearchResultsProps> = ({ results, onSelect }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedNode, setSelectedNode] =
    useState<string | undefined>(undefined);
  return (
    <Container>
      {results.map((result) => (
        <div key={uniqueId()} onClick={() => onSelect(result.id)}>
          <ListItem
            artwork={() => <Terminal size={20} color="#ff00cb" />}
            overrides={{
              Root: {
                style: {
                  backgroundColor: "transparent",
                  cursor: "pointer",
                  ":hover": {
                    backgroundColor: "rgba(99, 11, 226, 0.03)",
                  },
                },
              },
            }}
          >
            <ListItemLabel>
              <div>{result.name}</div>
              <Status>{startCase(result.status)}</Status>
            </ListItemLabel>
          </ListItem>
        </div>
      ))}
    </Container>
  );
};

export default SearchResults;
