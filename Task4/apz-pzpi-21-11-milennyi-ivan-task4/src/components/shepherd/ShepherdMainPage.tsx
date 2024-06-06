import { useState } from "react";

import { IUserProps } from "../properties/IUserProps";
import { useTranslation } from "react-i18next";
import { SheepList } from "./SheepList";
import { BreedsList } from "./BreedsList";
import { FeedsList } from "./FeedsList";

export const ShepherdMainPage: React.FC<IUserProps> = ({user}) =>{

    const {t} = useTranslation();
    const [selectedButton, setSelectedButton] = useState<number|null>(1);
    const [content, setContent] = useState<JSX.Element>(<SheepList id={user.Id!}/>);
    
    const handleButtonClick = (buttonIndex: number) => {
        setSelectedButton(buttonIndex);
        switch(buttonIndex) {
            case 1:
                setContent(<SheepList id={user.Id!}/>);
                break;
            case 2:
                setContent(<BreedsList/>);
                break;
            case 3:
                setContent(<FeedsList/>);
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
                        <h4 >Sheep</h4> 
                    </button>
                </div>
                <div>
                    <button style={{ backgroundColor: selectedButton === 2 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(2)}>
                        <h4 >Breeds</h4> 
                    </button>
                </div>
                <div>
                    <button style={{ backgroundColor: selectedButton === 3 ? 'rgb(238, 238, 238)' : 'white' }} onClick={() => handleButtonClick(3)}>
                        <h4 >Feeds</h4> 
                    </button>
                </div>
            </div>
            {content}
      </div>
    )
}