import { useEffect, useState } from "react";
import { BreedService } from "../../api/services/BreedService";
import { BreedVM } from "../../viewModels/BreedVM";

export const BreedsList = () =>{
    const breedService = new BreedService();
    const [breedsVMList, setBreedsVMList] = useState<BreedVM[]>([]);
    
    useEffect(() => {
        const fetchBreeds = async () =>{
            try{
                const data = await breedService.getAllVMs();
                setBreedsVMList(data);
            }
            catch (error){
                alert(error);
                setBreedsVMList([]);
            }
        }
        fetchBreeds();
    }, []);
    return (
        <div>
            {breedsVMList.length > 0 ? (breedsVMList.map((breed) => (
            <div key={breed.id}>
                <div>
                    <h4>{breed.name}</h4>
                </div>
                <div>
                    <h4>{breed.feedName}</h4>
                </div>
                <div>
                    <h4>{breed.sheepCount}</h4>
                </div>
                <div>
                    <h4>{breed.info}</h4>
                </div>
            </div>
          ))) : (
            <p></p>
          )}
        </div>
    );
}