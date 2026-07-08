use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

/// Initialise un nouveau OffsetPageTable.
///
/// Cette fonction est unsafe car l'appelant doit garantir que toute la
/// mémoire physique est bien mappée dans la mémoire virtuelle à partir de
/// `physical_memory_offset`. De plus, cette fonction ne doit être appelée
/// qu'une seule fois, pour éviter de créer des références `&mut` aliasées
/// (ce qui serait un comportement indéfini).
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Retourne une référence mutable vers la table de pages de niveau 4 active.
///
/// Cette fonction est unsafe car l'appelant doit garantir que toute la
/// mémoire physique est mappée dans la mémoire virtuelle à partir de
/// `physical_memory_offset`. De plus, cette fonction doit être appelée
/// une seule fois pour éviter les `&mut` aliasés.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

/// Crée un mapping d'exemple pour la page donnée vers la frame `0xb8000`
/// (le buffer VGA), à des fins de test.
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("map_to a échoué").flush();
}

/// Un FrameAllocator qui retourne les frames utilisables listées dans la
/// memory map fournie par le bootloader.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Crée un FrameAllocator à partir de la memory map passée en paramètre.
    ///
    /// Cette fonction est unsafe car l'appelant doit garantir que la memory
    /// map est valide : toutes les frames marquées `USABLE` doivent être
    /// réellement inutilisées.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Retourne un itérateur sur les frames utilisables de la memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // récupère les régions utilisables depuis la memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // transforme chaque région en plage d'adresses
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transforme en un itérateur d'adresses de début de frame
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // crée les `PhysFrame` à partir des adresses de début
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
