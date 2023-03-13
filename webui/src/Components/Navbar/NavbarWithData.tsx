import { FC } from "react";
import { useGetServicesQuery } from "../../Hooks/GraphQL";
import Navbar from "./Navbar";

const NavbarWithData: FC = () => {
  const { data } = useGetServicesQuery();
  const onSearch = (search: string) => {
    const results = data?.services.filter(
      (service) =>
        service.name.toLowerCase().includes(search.toLowerCase()) ||
        service.description.toLowerCase().includes(search.toLowerCase()) ||
        service.command.toLowerCase().includes(search.toLowerCase()) ||
        service.namespace.toLowerCase().includes(search.toLowerCase()) ||
        service.type.toLowerCase().includes(search.toLowerCase()) ||
        service.status.toLowerCase().includes(search.toLowerCase())
    );

    console.log(">> results", results);
  };
  return <Navbar onSearch={onSearch} />;
};

export default NavbarWithData;
