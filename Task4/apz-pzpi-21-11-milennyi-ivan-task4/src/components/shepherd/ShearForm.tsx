import { useNavigate, useParams } from "react-router-dom";
import { IUserProps } from "../properties/IUserProps"
import { useTranslation } from "react-i18next";
import { useEffectUser } from "../../utils/helpers";
import { useState } from "react";
import { ShearingLogService } from "../../api/services/ShearingLogService";
import { ShearingLog } from "../../models/ShearingLog";

export const ShearForm: React.FC<IUserProps> = ({user}) =>{
    const sheepId = parseInt(useParams().sheepId!);
    const shepherdId = user.Id!;
    const navigate = useNavigate();
    const {t} = useTranslation();
    const [woolAmount, setWoolAmount] = useState<number>(0);
    const [woolAmountError, setWoolAmountError] = useState<string>(""); 

    const shearingLogService = new ShearingLogService();

    useEffectUser(user, navigate);
    const createShearingLog = async () =>{
        if (woolAmount <= 0){
            setWoolAmountError("Wool amount must be greater than 0");
            return;
        }
        try{
            await shearingLogService.create(new ShearingLog(null, sheepId, shepherdId, new Date().getTime(), woolAmount));
            navigate("/sheep/" + sheepId + "/details");
        }
        catch(error){
            console.log(error);
        }
    }
    return(<button onClick={() => createShearingLog()}>Shear sheep</button> )
}