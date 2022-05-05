import { invoke } from '@tauri-apps/api';

export const saveCarryInfoData = async () => {
  return await invoke('save_carry_run_info', {
    data: 'hello tauri',
  });
};
