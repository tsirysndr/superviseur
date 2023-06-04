import { FC } from "react";
import NewServiceModal, { NewServiceModalProps } from "./NewServiceModal";
import { useRecoilState } from "recoil";
import { newServiceModalState } from "./NewServiceModalState";

export type NewServiceModalWithDataProps = {
  isOpen: boolean;
  onClose: () => void;
};

const NewServiceModalWithData: FC<NewServiceModalWithDataProps> = (props) => {
  const [{ templates, databases }, setState] =
    useRecoilState(newServiceModalState);
  return (
    <NewServiceModal {...props} templates={templates} databases={databases} />
  );
};

export default NewServiceModalWithData;
