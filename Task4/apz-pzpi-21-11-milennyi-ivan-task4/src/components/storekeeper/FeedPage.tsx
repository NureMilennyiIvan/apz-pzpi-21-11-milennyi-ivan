import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { useEffectUser } from "../../utils/helpers";
import { useState } from "react";
import { IUserProps } from "../properties/IUserProps";
import { FeedingLogsListFeed } from "./FeedingLogsListFeed";
import { FeedSuppliesList } from "./FeedSuppliesList";
import styles from "../../assets/css/FeedPage.module.css";

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
                setContent(<FeedingLogsListFeed feedId={parseInt(feedId!)}/>);
                break;
            default:
                break;
        }
    }
    
    return(
        <div className={styles.container}>
            <div className={styles.buttonPanel}>
                    <button className={styles.button} style={{ backgroundColor: selectedButton === 1 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(1)}>
                        <h4 >Supplies</h4> 
                    </button>

                    <button className={styles.button} style={{ backgroundColor: selectedButton === 2 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(2)}>
                        <h4 >Consumptions</h4> 
                    </button>
            </div>
            <div className={styles.content}>
                {content}
            </div>
      </div>
    )
}