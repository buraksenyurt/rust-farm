import {WorkItemManager} from './pkg/can_ban.js';

export async function createWorkItem(workItemData) {
    const manager = WorkItemManager.new();
    if(manager.items_count>5){
        console.log("There can be only 5(Five) Task at board...");

    }
    console.log(workItemData);
    const payload = await manager
        .create(
            workItemData.title
            , workItemData.duration
            , workItemData.durationType
            , workItemData.size);
    return JSON.parse(payload);
}

export async function fetchWorkItems() {
    const manager = WorkItemManager.new();
    try {
        const jsStringResponse = await manager.get_board();
        return JSON.parse(jsStringResponse);
    } catch (error) {
        console.error("Error fetching board items:", error);
        throw error;
    }
}

export async function getWorkItemsCount() {
    const manager = WorkItemManager.new();
    try {
        const jsStringResponse = await manager.get_items_count()
        return JSON.parse(jsStringResponse);
    } catch (error) {
        console.error("Error fetching board items:", error);
        throw error;
    }
}
