import { useNavigate, useParams } from "react-router-dom";
import { IUserProps } from "../properties/IUserProps";
import { useTranslation } from "react-i18next";
import { useEffectUser } from "../../utils/helpers";

export const FeedForm: React.FC<IUserProps> = ({user}) =>{
    const sheepId = parseInt(useParams().sheepId!);
    const shepherdId = user.Id!;
    const navigate = useNavigate();
    const {t} = useTranslation();
    useEffectUser(user, navigate);
    return(<></>)
}