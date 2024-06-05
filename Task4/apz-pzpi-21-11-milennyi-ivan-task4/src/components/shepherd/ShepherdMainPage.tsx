import { useEffect, useState } from "react";
import { SheepService } from "../../api/services/SheepService";
import { SheepVM } from "../../viewModels/SheepVM";
import { IUserProps } from "../properties/IUserProps";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

export const ShepherdMainPage: React.FC<IUserProps> = ({user}) =>{
    const sheepService = new SheepService();
    const [sheepVM, setSheepVM] = useState<SheepVM[]>([]);
    const {t} = useTranslation();
    const navigate = useNavigate();

    useEffect(() => {
        const fetchSheep = async () =>{
            try{
                const data = await sheepService.getAllVMsByShepherdId(user.Id!);
                setSheepVM(data);
            }
            catch (error){
                alert(error);
                setSheepVM([]);
            }
        }
        fetchSheep();
    }, []);
    return(
        <div>
        <div>
            {sheepVM.length > 0 ? (sheepVM.map((sheep) => (
            <div key={sheep.id} onClick={() => navigate("sheep/" + sheep.id + "/details")}>
                <div>
                    <h4>{sheep.id}</h4>
                </div>
                <div>
                    <h4>{sheep.breed}</h4>
                </div>
                <div>
                    <h4>{sheep.sex}</h4>
                </div>
                <div>
                    <h4>{sheep.lastFeedingDate}</h4>
                </div>
                <div>
                    <h4>{sheep.lastFeedingDate}</h4>
                </div>
            </div>
          ))) : (
            <p></p>
          )}
        </div>
      </div>
    )
}