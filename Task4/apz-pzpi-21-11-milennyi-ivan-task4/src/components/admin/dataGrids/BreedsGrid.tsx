import { useEffect, useState } from "react";
import { BreedService } from "../../../api/services/BreedService";
import styles from "../../../assets/css/GridComponent.module.css"
import { Breed } from "../../../models/Breed";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

export const BreedsGrid = () =>{
    const breedService = new BreedService();
    const [selectedBreed, setSelectedBreed] = useState<Breed | null>(null);
    const [breeds, setBreeds] = useState<Breed[]>([]);
    const [trigger, setTrigger] = useState<boolean>(true);

    const {t} = useTranslation();
    const navigate = useNavigate();
    
    useEffect(() => {
        const fetchBreeds = async () => {
          try {
            const data = await breedService.getAll();
            setBreeds(data);
          } catch (error) {
            alert(error);
            setBreeds([]);
          }
        };
        fetchBreeds();
    }, [trigger]);

    const handleRowSelection = (breed: Breed) => {
        setSelectedBreed(breed);
    };
    const editBreed = (id: number) => {
        //navigate("edit/city/" + (selectedCity?.Id as number));
    }
    const deleteBreed = async (id: number) => {
        try{
            await breedService.delete(id);
            setSelectedBreed(null);
            setTrigger(!trigger);
        }
        catch(error){
            console.log(error);
            alert("Error");
        }
    }
  
    return (
        <div className={styles.container}>
            <table className={styles.table}>
                <thead>
                    <tr className={styles.tr}>
                        <th className={styles.th}>{t("gridBase.chooseHeader")}</th>
                        <th className={styles.th}>{t("gridBase.idHeader")}</th>
                        <th className={styles.th}>{t("breedsGrid.nameHeader")}</th>
                        <th className={styles.th}>{t("breedsGrid.feedIdHeader")}</th>
                        <th className={styles.th}>{t("breedsGrid.infoHeader")}</th>
                    </tr>
                </thead>
                <tbody>
                    {breeds.length != 0 ? (
                        breeds.map((breed) => (
                            <tr key={breed.id} 
                                className={`${styles.tr} ${selectedBreed?.id === breed.id ? styles.selected : ''}`}>
                                <td className={styles.td}>
                                    <input
                                        type="radio"
                                        name="selectedUser"
                                        checked={selectedBreed?.id === breed.id}
                                        onChange={() => handleRowSelection(breed)}
                                    />
                                </td>
                                <td className={styles.td}>{breed.id}</td>
                                <td className={styles.td}>{breed.name}</td>
                                <td className={styles.td}>{breed.feed_id}</td>
                                <td className={styles.td}>{breed.info}</td>
                            </tr>
                        ))
                    ) : (
                        <div className={styles.error}>{t("cityGrid.errorHeader")}</div>
                    )}
                </tbody>
            </table>
            <div className={styles.actionButtonsContainer}>
                <button className={styles.actionButton} onClick={() => navigate("/shepherd/create")}>
                    {t("gridBase.addButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedBreed?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedBreed ? editBreed : undefined}
                    style={{ cursor: selectedBreed ? 'pointer' : 'default' }}>
                    {t("cityGrid.editButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedBreed?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedBreed ? deleteBreed(selectedBreed.id!) : undefined}
                    style={{ cursor: selectedBreed ? 'pointer' : 'default' }}>
                    {t("cityGrid.deleteButtonText")}
                </button>
            </div>
        </div>
    );
}