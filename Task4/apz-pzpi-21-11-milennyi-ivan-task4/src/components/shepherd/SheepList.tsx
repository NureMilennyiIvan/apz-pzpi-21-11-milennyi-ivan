import { useEffect, useState } from "react";
import { SheepService } from "../../api/services/SheepService";
import { SheepVM } from "../../viewModels/SheepVM";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";

interface ISheepList{
    id: number;
}

export const SheepList: React.FC<ISheepList> = ({id}) =>{
    const sheepService = new SheepService();
    const [sheepVMList, setSheepVMList] = useState<SheepVM[]>([]);
    const navigate = useNavigate();
    const {t} = useTranslation();
    
    useEffect(() => {
        const fetchSheep = async () =>{
            try{
                const data = await sheepService.getAllVMsByShepherdId(id);
                setSheepVMList(data);
            }
            catch (error){
                alert(error);
                setSheepVMList([]);
            }
        }
        fetchSheep();
    }, []);
    return (
        <div>
            {sheepVMList.length > 0 ? (sheepVMList.map((sheep) => (
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
    );
}
