import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { useEffectUser } from "../../utils/helpers";
import { useState } from "react";
import { IUserProps } from "../properties/IUserProps";
import { FeedingLogsList } from "./FeedingLogsList";
import { FeedSuppliesList } from "./FeedSuppliesList";

export const FeedPage: React.FC<IUserProps> = ({user}) =>{
    const [selectedButton, setSelectedButton] = useState<number>(1);
    const { feedId } = useParams();
    const [content, setContent] = useState<JSX.Element>(<FeedSuppliesList feedId={parseInt(feedId!)}/>);
    const {t} = useTranslation();
    const navigate = useNavigate();
    useEffectUser(user, navigate);
    
    const handleButtonClick = (buttonIndex: number) => {
        setSelectedButton(buttonIndex);
        switch(buttonIndex) {
            case 1:
                setContent(<FeedSuppliesList feedId={parseInt(feedId!)}/>);
                break;
            case 2:
                setContent(<FeedingLogsList feedId={parseInt(feedId!)}/>);
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
                        <h4 >Supplies</h4> 
                    </button>
                </div>
                <div>
                    <button style={{ backgroundColor: selectedButton === 2 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(2)}>
                        <h4 >Consumptions</h4> 
                    </button>
                </div>
            </div>
            {content}
      </div>
    )
}