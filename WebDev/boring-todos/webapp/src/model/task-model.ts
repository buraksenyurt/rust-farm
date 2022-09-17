import {get,post,patch,del} from '../http_client';
import {hub} from 'dom-native';

export interface Task{
    id:number,
    title:string;
    state : 'Ready'|'InProgress'|'Completed';
}

export type TaskDao = Partial<Omit<Task,'id'>>;

// Model Access Coordinator
class TaskMac {

    // Tüm görevlerin çekilmesini sağlayan fonksiyon
    async getAllTasks() : Promise<Task[]> {
        const taskList = await get("tasks");
        return taskList as Task[];
    }

    // Yeni bir görev oluşturmak için kullanılan fonksiyon
    async createTask(data:TaskDao) : Promise<Task> {
        // Şimdilik gelen verinin title bilgisini kontrol ediyoruz.
        // Daha gelişmiş bir doğrulama sistemi entegre edilebilir
        if(data.title == null || data.title.trim().length == 0){
            throw new Error("Görev başlığı girilmemiş");
        }
        // servis tarafına post çağrısı yapıyoruz.
        const created = await post(`tasks`,data);

        // client-side message broker olan hub nesnesinden yararlanarak
        // Task isimli bir topic altında create başlığında(label) yeni bir task
        // nesnesinin oluştuğunu taskHub isimli olay çerçevesinde yayınlıyoruz(publish).
        // Bu event hub yardımıyla diğer bileşenleri kolayca haberdar edebiliriz.
        hub('taskHub').pub('Task','create',created);

        return created as Task;
    }

    // Bir görevi güncellemek için kullanılan fonksiyon.
    async updateTask(task_id:number,data:TaskDao): Promise<Task>{
        const updated = await patch(`tasks/${task_id}`,data);
        hub('taskHub').pub('Task','update',updated);
        return updated as Task;
    }

    // Bu da bir görevi silmek için kullanılan fonksiyon
    async deleteTask(task_id:number): Promise<Task>{
        const deleted = await del(`tasks/${task_id}`);
        hub('taskHub').pub('Task','delete',deleted);
        return deleted as Task;
    }
}

export const taskMac = new TaskMac();