import { useEffect, useState } from "react";
import { BreedService } from "../../api/services/BreedService";
import { BreedVM } from "../../viewModels/BreedVM";
import styles from '../../assets/css/BreedList.module.css';

export const BreedsList = () => {
    const breedService = new BreedService();
    const [breedsVMList, setBreedsVMList] = useState<BreedVM[]>([]);
    
    useEffect(() => {
        const fetchBreeds = async () => {
            try {
                const data = await breedService.getAllVMs();
                setBreedsVMList(data);
            } catch (error) {
                alert(error);
                setBreedsVMList([]);
            }
        }
        fetchBreeds();
    }, []);
    
    return (
        <div className={styles.list}>
            {breedsVMList.length > 0 ? (
                breedsVMList.map((breed) => (
                    <div key={breed.id} className={styles.card}>
                        <div className={styles.cardHeader}>
                            <h2 className={styles.breedName}>{breed.name}</h2>
                        </div>
                        <div className={styles.cardBody}>
                            <p><strong>Корм:</strong> {breed.feedName}</p>
                            <p><strong>Кількість овець:</strong> {breed.sheepCount}</p>
                            <p><strong>Інформація:</strong> {breed.info}</p>
                        </div>
                    </div>
                ))
            ) : (
                <p>Немає даних про породи</p>
            )}
        </div>
    );
}
