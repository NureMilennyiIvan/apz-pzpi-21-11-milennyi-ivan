import { useEffect, useState } from "react";
import { SheepService } from "../../api/services/SheepService";
import { IUserProps } from "../properties/IUserProps";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { SheepDetailsVM } from "../../viewModels/extraViewModels/SheepDetailsVM";
import { useEffectUser } from "../../utils/helpers";
import { FeedingLog } from "../../models/FeedingLog";
import { FeedingLogService } from "../../api/services/FeedingLogService";
import { ShearingLog } from "../../models/ShearingLog";
import { ShearingLogService } from "../../api/services/ShearingLogService";

interface ISheepDetailsPage{
    shepherdId: number;
    sheepId: number;
}
export const SheepDetailsPage: React.FC<ISheepDetailsPage> = ({shepherdId, sheepId}) =>{
    const sheepService = new SheepService();
    const feedingLogService = new FeedingLogService();
    const shearingLogService = new ShearingLogService();
    const [sheepDetails, setSheepDetails] = useState<SheepDetailsVM | null>();
    const [trigger, setTrigger] = useState<boolean>(true);

    const [woolAmount, setWoolAmount] = useState<string>(''); 
    const [errorWoolAmount, setErrorWoolAmount] = useState<string>('');
    const [errorAmount, setErrorAmount] = useState<string>('');
    const {t} = useTranslation();



    useEffect(() => {
        const fetchSheepDetails = async () =>{
            try{
                const data = await sheepService.getDetailsById(sheepId);
                setErrorAmount('');
                setErrorWoolAmount('');
                setWoolAmount('');
                setSheepDetails(data);
            }
            catch (error){
                alert(error);
                setSheepDetails(null);
            }
        }
        fetchSheepDetails();
    }, [trigger]);

    const createFeedingLog = async (details: SheepDetailsVM) =>{
        if (details.requiredFeedAmount <= 0){
            setErrorAmount(t(""));
            return;
        }
        setErrorAmount('');
        try{
            const feedingLog = new FeedingLog(null, sheepId, shepherdId, new Date().getTime(), details.feedId, details.requiredFeedAmount * 1000);
            await feedingLogService.create(feedingLog);
            setTrigger(!trigger);
        }
        catch(error){
            console.log(error);
            setErrorAmount(t(""));
        }
    }
    const createShearingLog = async () =>{
        if (woolAmount.length == 0 || !(/^(0|[1-9]\d*)$/.test(woolAmount))) {
            setErrorWoolAmount(t(""));
            return;
        }
        setErrorWoolAmount('');
        try{
            const shearingLog = new ShearingLog(null, sheepId, shepherdId, new Date().getTime(), parseInt(woolAmount) * 1000);
            await shearingLogService.create(shearingLog);
            setTrigger(!trigger);
        }
        catch(error){
            console.log(error);
            setErrorWoolAmount(t(""));
        }
    };
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
                        <h4>{sheepDetails.requiredFeedAmount}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.availableFeedAmount}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.lastFeedingDate ? sheepDetails.lastFeedingDate : "No date"}</h4>
                    </div>
                    <div>
                        <h4>{sheepDetails.lastShearingDate ? sheepDetails.lastShearingDate : "No date"}</h4>
                    </div>

                    <div>
                        {sheepDetails.isFeed ? (
                            <button onClick={() => createFeedingLog(sheepDetails)}>Feed sheep</button> 
                        ):     (
                            <></>
                        )}
                    </div>
                    <div>
                        {sheepDetails.isShear ? (
                            <button onClick={() => createShearingLog()}>Shear sheep</button> 
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