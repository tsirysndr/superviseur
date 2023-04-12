import { FC, useState } from "react";
import { Service, useGetServicesQuery } from "../../Hooks/GraphQL";
import Navbar from "./Navbar";
import { useParams } from "react-router-dom";

const NavbarWithData: FC = () => {
  const [results, setResults] = useState<Service[]>([]);
  const { projectId } = useParams();
  const { data } = useGetServicesQuery({
    variables: { projectId: projectId! },
  });
  const onSearch = (search: string) => {
    if (search === "") {
      setResults([]);
      return;
    }
    setResults(
      data?.services.filter(
        (service) =>
          service.name.toLowerCase().includes(search.toLowerCase()) ||
          service.description!.toLowerCase().includes(search.toLowerCase()) ||
          service.command.toLowerCase().includes(search.toLowerCase()) ||
          service.namespace.toLowerCase().includes(search.toLowerCase()) ||
          service.type.toLowerCase().includes(search.toLowerCase()) ||
          service.status.toLowerCase().includes(search.toLowerCase())
      ) || []
    );
  };
  return <Navbar onSearch={onSearch} results={results!} />;
};

export default NavbarWithData;
