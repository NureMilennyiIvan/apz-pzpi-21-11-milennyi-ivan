import { useEffect, useState } from "react";
import { SheepService } from "../../api/services/SheepService";
import { IUserProps } from "../properties/IUserProps";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { SheepDetailsVM } from "../../viewModels/extraViewModels/SheepDetailsVM";
import { useEffectUser } from "../../utils/helpers";

export const SheepDetailsPage: React.FC<IUserProps> = ({user}) =>{
    const sheepService = new SheepService();
    const [sheepDetails, setSheepDetails] = useState<SheepDetailsVM | null>();
    const { sheepId } = useParams();
    const {t} = useTranslation();
    const navigate = useNavigate();
    useEffectUser(user, navigate);

    useEffect(() => {
        const fetchSheepDetails = async () =>{
            try{
                const data = await sheepService.getDetailsById(parseInt(sheepId!));
                setSheepDetails(data);
            }
            catch (error){
                alert(error);
                setSheepDetails(null);
            }
        }
        fetchSheepDetails();
    }, []);
    return(
        <div>
        <div>
            {sheepDetails ? (
                <div>
                    <div>
                        <h4>{sheepDetails.id}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.breed}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.breedInfo}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.age}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.sex}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.temperature}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.weight}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.feedId}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.feedName}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.feedAmount}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.lastFeedingDate ? sheepDetails.lastFeedingDate : "No date"}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.lastShearingDate ? sheepDetails.lastShearingDate : "No date"}</h4>
                    </div>

                    <div>
                        {sheepDetails.isFeed ? (
                            <button onClick={() => navigate("/sheep/" + sheepId + "/create/feeding-log")}>Feed sheep</button> 
                        ):     (
                            <></>
                        )}
                    </div>
                    <div>
                        {sheepDetails.isShear ? (
                            <button onClick={() => navigate("/sheep/" + sheepId + "/create/shearing-log")}>Feed sheep</button> 
                        ):     (
                            <></>
                        )}
                    </div>
                </div>
            ) : (
                <></>
            )}
        </div>
      </div>
    )
}