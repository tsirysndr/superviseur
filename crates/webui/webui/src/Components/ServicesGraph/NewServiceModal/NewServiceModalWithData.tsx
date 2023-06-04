import { FC } from "react";
import NewServiceModal from "./NewServiceModal";
import { useRecoilState } from "recoil";
import { newServiceModalState } from "./NewServiceModalState";

export type NewServiceModalWithDataProps = {
  isOpen: boolean;
  onClose: () => void;
};

const NewServiceModalWithData: FC<NewServiceModalWithDataProps> = (props) => {
  const [{ templates, databases, messaging }] =
    useRecoilState(newServiceModalState);
  return (
    <NewServiceModal
      {...props}
      templates={templates}
      databases={databases}
      messaging={messaging}
    />
  );
};

export default NewServiceModalWithData;
