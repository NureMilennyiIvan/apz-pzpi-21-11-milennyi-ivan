import { useNavigate, useParams } from "react-router-dom";
import { IUserProps } from "../properties/IUserProps";
import { useTranslation } from "react-i18next";
import { useEffectUser } from "../../utils/helpers";
import { SheepDetailsPage } from "./SheepDetailsPage";
import { useState } from "react";
import { FeedingLogsListSheep } from "./FeedingLogsListSheep";
import { ShearingLogsList } from "./ShearingLogsList";

export const SheepMainPage: React.FC<IUserProps> = ({user}) =>{
    const sheepId = parseInt(useParams().sheepId!);
    const [selectedButton, setSelectedButton] = useState<number>(1);
    const [content, setContent] = useState<JSX.Element>(<SheepDetailsPage shepherdId={user.Id!} sheepId={sheepId} />);

    const {t} = useTranslation();
    const navigate = useNavigate();
    useEffectUser(user, navigate);


    
    const handleButtonClick = (buttonIndex: number) => {
        setSelectedButton(buttonIndex);
        switch(buttonIndex) {
            case 1:
                setContent(<SheepDetailsPage shepherdId={user.Id!} sheepId={sheepId} />);
                break;
            case 2:
                setContent(<FeedingLogsListSheep sheepId={sheepId}/>);
                break;
            case 3:
                setContent(<ShearingLogsList sheepId={sheepId}/>);
                break;
            default:
                break;
        }
    }
    return(
        <div>
            <div>
                <div>
                    <button style={{ backgroundColor: selectedButton === 1 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(1)}>
                        <h4 >Sheep details</h4> 
                    </button>
                </div>
                <div>
                    <button style={{ backgroundColor: selectedButton === 2 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(2)}>
                        <h4 >Feeding logs</h4> 
                    </button>
                </div>
                <div>
                    <button style={{ backgroundColor: selectedButton === 3 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(3)}>
                        <h4 >Shearing logs</h4> 
                    </button>
                </div>
            </div>
            {content}
        </div>
    )
}