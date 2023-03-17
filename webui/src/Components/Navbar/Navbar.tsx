import { FC, useEffect, useState } from "react";
import { Input, SIZE } from "baseui/input";
import styled from "@emotion/styled";
import { SearchOutline } from "@styled-icons/evaicons-outline/SearchOutline";
import { Github } from "@styled-icons/evaicons-solid/Github";
import { Feedback } from "@styled-icons/remix-line/Feedback";
import { useForm, Controller } from "react-hook-form";
import { Popover } from "baseui/popover";
import SearchResults from "./SearchResults";
import { Service } from "../../Hooks/GraphQL";
import ServiceDetails from "../ServiceDetails";
import { Drawer } from "baseui/drawer";

const Container = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  background-color: #630be2;
  height: 40px;
`;

const Logo = styled.div`
  color: #fff;
  font-weight: bold;
  width: 130px;
  text-align: center;
`;

const Settings = styled.div`
  width: 200px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-end;
  padding-right: 25px;
`;

interface NavbarProps {
  onSearch: (value: string) => void;
  results: Service[];
}

const Navbar: FC<NavbarProps> = ({ onSearch, results }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedNode, setSelectedNode] =
    useState<string | undefined>(undefined);
  const { control, watch, reset } = useForm();
  useEffect(() => {
    const subscription = watch(
      (value, { name }) => name && onSearch(value[name])
    );
    return () => subscription.unsubscribe();
  }, [watch, onSearch]);
  return (
    <Container>
      <Logo>Superviseur</Logo>
      <Controller
        render={({ field }) => (
          <Popover
            isOpen={results && results.length > 0}
            content={() => (
              <SearchResults
                results={results}
                onSelect={(id) => {
                  onSearch("");
                  reset({
                    search: "",
                  });
                  setSelectedNode(id);
                  setIsOpen(true);
                }}
              />
            )}
            overrides={styles.popover}
          >
            <div>
              <Input
                {...(field as any)}
                placeholder="Search for a service ..."
                clearable
                clearOnEscape
                size={SIZE.default}
                startEnhancer={() => (
                  <SearchOutline size={20} color="#f9f9f9c6" />
                )}
                overrides={styles.input}
              />
            </div>
          </Popover>
        )}
        control={control}
        name="search"
        rules={{ required: true }}
      />

      <Settings>
        <a
          href="https://github.com/tsirysndr/superviseur/issues/new"
          target="_blank"
          rel="noreferrer"
          style={{ marginRight: "20px" }}
          title="Report a bug"
        >
          <Feedback size={24} color="#fff" />
        </a>
        <a
          href="https://github.com/tsirysndr/superviseur"
          target="_blank"
          rel="noreferrer"
          title="View on Github"
        >
          <Github size={24} color="#fff" />
        </a>
      </Settings>

      <Drawer
        isOpen={isOpen}
        autoFocus
        onClose={() => {
          setIsOpen(false);
        }}
        overrides={{
          Root: {
            style: {
              zIndex: 1,
            },
          },
          DrawerContainer: {
            style: {
              width: "45vw",
            },
          },
        }}
      >
        <ServiceDetails selectedNode={selectedNode} />
      </Drawer>
    </Container>
  );
};

const styles = {
  input: {
    Root: {
      style: {
        width: "400px",
        height: "35px",
        borderWidth: "0px",
        borderRadius: "2px",
        backgroundColor: "#5a10c5",
      },
    },
    Input: {
      style: {
        color: "#fff",
        caretColor: "#fff",
        "::placeholder": {
          color: "#f9f9f990",
        },
        ":-ms-input-placeholder": {
          color: "#f9f9f990",
        },
        "::-ms-input-placeholder": {
          color: "#f9f9f990",
        },
      },
    },
    InputContainer: {
      style: {
        backgroundColor: "#5a10c5",
      },
    },
    StartEnhancer: {
      style: {
        paddingLeft: "0px",
        backgroundColor: "#5a10c5",
      },
    },
    ClearIconContainer: {
      style: {
        color: "#fff",
      },
    },
  },
  popover: {
    Inner: {
      style: {
        backgroundColor: "#fff",
        borderTopLeftRadius: "0px",
        borderTopRightRadius: "0px",
        top: "-1px",
      },
    },
    Body: {
      style: {
        top: "-2px",
      },
    },
  },
};

export default Navbar;
