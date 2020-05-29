use ::libc;
extern "C" {
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
    /* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
    /* ***************************************************************************************
 *
 * Basic debugging macro's
 *
 */
    /*
 *
 * ASSERT
 *
 * Rewritten version of assert that allows a printf format text string to be passed
 * to ASSERT along with the condition.
 *
 * Arguments:	ASSERT( condition, "Format string with variables: %d, %d", var1, var2 );
 */
    /* ***************************************************************************************
 *
 * Conditional debugging macro's that can be selectively turned on or off on a file
 * by file basis.
 *
 * Modified to not output nothing under no conditions
 *
 */
    /* **
 ***
 ***  New debug logging output interface below. Heavily inspired
 ***  by similar code in Freeciv. Parts ripped directly.
 ***
 ***/
    /* Want to use GCC's __attribute__ keyword to check variadic
 * parameters to printf-like functions, without upsetting other
 * compilers: put any required defines magic here.
 * If other compilers have something equivalent, could also
 * work that out here.   Should this use configure stuff somehow?
 * --dwp
 */
    /* Must match code_part_names in debug.c */
    /* special: sets all to on */
    /* special; on by default */
    /* if too verbose for anything but dedicated debugging... */
    /* _must_ be last! */
    // / Function which does the output
    // / Setup function
    // / Cleaning function
    // / Used to pass data to the above functions. Eg a filename or handle.
    /* *
 * Call once to initialize the debug logging system.
 *
 * Doesn't register any callbacks!
 */
    /* *
 * Shutdown the debug system and remove all output callbacks
 */
    /* *
 * Register a callback to be called on every call to debug()
 *
 * \param	callback	Function which does the output
 * \param	init		Initializer function which does all setup for the callback (optional, may be NULL)
 * \param	exit		Cleanup function called when unregistering the callback (optional, may be NULL)
 * \param	data		Data to be passed to all three functions (optional, may be NULL)
 */
    /* *
 * Toggle debug output for part associated with str
 *
 * \param	str	Codepart in textformat
 */
    /* *
 * Output printf style format str with additional arguments.
 *
 * Only outputs if debugging of part was formerly enabled with debug_enable_switch.
 *
 * \param	part	Code part to associate with this message
 * \param	str		printf style formatstring
 */
    #[no_mangle]
    fn debug(part: code_part, str: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_uint;
pub type BOOL = libc::c_int;
pub type code_part = libc::c_uint;
pub const LOG_LAST: code_part = 12;
pub const LOG_SCRIPT: code_part = 11;
pub const LOG_NEVER: code_part = 10;
pub const LOG_ERROR: code_part = 9;
pub const LOG_MEMORY: code_part = 8;
pub const LOG_NET: code_part = 7;
pub const LOG_TEXTURE: code_part = 6;
pub const LOG_3D: code_part = 5;
pub const LOG_WZ: code_part = 4;
pub const LOG_VIDEO: code_part = 3;
pub const LOG_SOUND: code_part = 2;
pub const LOG_MAIN: code_part = 1;
pub const LOG_ALL: code_part = 0;
/* **************************************************************************/
/*
 * Priority queue code
 *
 */
/* **************************************************************************/
/* **************************************************************************/
pub type QUEUE_CLEAR_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE_NODE {
    pub psElement: *mut libc::c_void,
    pub iPriority: libc::c_int,
    pub psPrev: *mut QUEUE_NODE,
    pub psNext: *mut QUEUE_NODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub psFreeNodeList: *mut QUEUE_NODE,
    pub psNodeQHead: *mut QUEUE_NODE,
    pub psCurNode: *mut QUEUE_NODE,
    pub iElementSize: libc::c_int,
    pub iMaxElements: libc::c_int,
    pub iFreeNodes: libc::c_int,
    pub iQueueNodes: libc::c_int,
    pub pfClearFunc: QUEUE_CLEAR_FUNC,
}
/* **************************************************************************/
/* **************************************************************************/
/*
 * Priority queue implementation
 *
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_Init(mut ppQueue: *mut *mut QUEUE,
                                    mut iMaxElements: libc::c_int,
                                    mut iElementSize: libc::c_int,
                                    mut pfClearFunc: QUEUE_CLEAR_FUNC)
 -> BOOL {
    let mut i: libc::c_int = 0;
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    /* allocate queue */
    *ppQueue =
        memMallocRelease(::std::mem::size_of::<QUEUE>() as libc::c_ulong) as
            *mut QUEUE;
    if (*ppQueue).is_null() {
        debug(LOG_ERROR,
              b"queue_Init: couldn\'t allocate memory for queue\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    /* set up free node list */
    (**ppQueue).psFreeNodeList = 0 as *mut QUEUE_NODE;
    i = 0 as libc::c_int;
    while i < iMaxElements {
        /* allocate node */
        psNode =
            memMallocRelease(::std::mem::size_of::<QUEUE_NODE>() as
                                 libc::c_ulong) as *mut QUEUE_NODE;
        if (*ppQueue).is_null() {
            debug(LOG_ERROR,
                  b"queue_Init: couldn\'t allocate memory for queue node\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        /* init node */
        (*psNode).psElement = 0 as *mut libc::c_void;
        (*psNode).iPriority = 0 as libc::c_int;
        /* add to head of list */
        (*psNode).psPrev = 0 as *mut QUEUE_NODE;
        (*psNode).psNext = (**ppQueue).psFreeNodeList;
        (**ppQueue).psFreeNodeList = psNode;
        i += 1
    }
    /* init node counts */
    (**ppQueue).iFreeNodes = iMaxElements;
    (**ppQueue).iQueueNodes = 0 as libc::c_int;
    /* init queue list */
    (**ppQueue).psNodeQHead = 0 as *mut QUEUE_NODE;
    (**ppQueue).psCurNode = 0 as *mut QUEUE_NODE;
    /* init other queue struct members */
    (**ppQueue).iElementSize = iElementSize;
    (**ppQueue).iMaxElements = iMaxElements;
    (**ppQueue).pfClearFunc = pfClearFunc;
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_Destroy(mut pQueue: *mut QUEUE) {
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    /* free up free node list */
    while !(*pQueue).psFreeNodeList.is_null() {
        psNode = (*pQueue).psFreeNodeList;
        (*pQueue).psFreeNodeList = (*(*pQueue).psFreeNodeList).psNext;
        memFreeRelease(psNode as *mut libc::c_void);
        psNode = 0 as *mut QUEUE_NODE
    }
    /* free up node queue */
    while !(*pQueue).psNodeQHead.is_null() {
        psNode = (*pQueue).psNodeQHead;
        (*pQueue).psNodeQHead = (*(*pQueue).psNodeQHead).psNext;
        memFreeRelease(psNode as *mut libc::c_void);
        psNode = 0 as *mut QUEUE_NODE
    }
    memFreeRelease(pQueue as *mut libc::c_void);
    pQueue = 0 as *mut QUEUE;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_Clear(mut pQueue: *mut QUEUE) {
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    while !(*pQueue).psNodeQHead.is_null() {
        psNode = (*pQueue).psNodeQHead;
        (*pQueue).psNodeQHead = (*(*pQueue).psNodeQHead).psNext;
        /* do element callback */
        (*pQueue).pfClearFunc.expect("non-null function pointer")((*psNode).psElement);
        /* return node to free node list */
        (*psNode).psNext = (*pQueue).psFreeNodeList;
        (*pQueue).psFreeNodeList = psNode;
        /* update node counts */
        (*pQueue).iFreeNodes += 1;
        (*pQueue).iQueueNodes -= 1
    };
}
/* **************************************************************************/
/*
 * queue_Enqueue
 *
 * Add element to queue in priority order
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_Enqueue(mut pQueue: *mut QUEUE,
                                       mut psElement: *mut libc::c_void,
                                       mut iPriority: libc::c_int) {
    let mut psFreeNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    let mut psNodePrev: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    /* check input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_Enqueue: queue pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              141 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"queue_Enqueue\x00")).as_ptr(),
              b"PTRVALID(pQueue,sizeof(QUEUE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_Enqueue: element pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              143 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"queue_Enqueue\x00")).as_ptr(),
              b"PTRVALID(psElement,pQueue->iElementSize)\x00" as *const u8 as
                  *const libc::c_char);
    };
    psNode = (*pQueue).psNodeQHead;
    while !psNode.is_null() {
        if (*psNode).psElement != psElement {
        } else {
            debug(LOG_ERROR,
                  b"duplicate element found\n\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psNode).psElement != psElement {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"pqueue.c\x00" as *const u8 as *const libc::c_char,
                  150 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"queue_Enqueue\x00")).as_ptr(),
                  b"psNode->psElement != psElement\x00" as *const u8 as
                      *const libc::c_char);
        };
        psNode = (*psNode).psNext
    }
    /* check list not empty */
    if (*pQueue).psFreeNodeList.is_null() {
        debug(LOG_NEVER,
              b"queue_GetFreeElement: all nodes allocated: flushing queue.\n\x00"
                  as *const u8 as *const libc::c_char);
        queue_Clear(pQueue);
    }
    /* get free node from head of list */
    psFreeNode = (*pQueue).psFreeNodeList;
    (*pQueue).psFreeNodeList = (*(*pQueue).psFreeNodeList).psNext;
    /* update node counts */
    (*pQueue).iFreeNodes -= 1;
    (*pQueue).iQueueNodes += 1;
    /* attach element to node */
    (*psFreeNode).psElement = psElement;
    /* init pointer to head of queue */
    psNode = (*pQueue).psNodeQHead;
    psNodePrev = 0 as *mut QUEUE_NODE;
    /* init node priority */
    (*psFreeNode).iPriority = iPriority;
    /* find correct place in queue according to priority */
    while !psNode.is_null() {
        /* break if priority slot found */
        if (*psNode).iPriority < iPriority { break ; }
        /* update pointers */
        psNodePrev = psNode;
        psNode = (*psNode).psNext
    }
    /* insert new node */
    (*psFreeNode).psPrev = psNodePrev;
    (*psFreeNode).psNext = psNode;
    /* update previous queue node pointer */
    if psNodePrev.is_null() {
        (*pQueue).psNodeQHead = psFreeNode
    } else { (*psNodePrev).psNext = psFreeNode }
    /* update next queue node pointer */
    if !psNode.is_null() { (*psNode).psPrev = psFreeNode };
}
/* **************************************************************************/
/*
 * queue_Dequeue
 *
 * Remove head of queue
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_Dequeue(mut pQueue: *mut QUEUE)
 -> *mut libc::c_void {
    let mut psElement: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    /* check input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_Dequeue: queue pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              232 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"queue_Dequeue\x00")).as_ptr(),
              b"PTRVALID(pQueue,sizeof(QUEUE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(*pQueue).psNodeQHead.is_null() {
        /* remove node at head of queue */
        psNode = (*pQueue).psNodeQHead;
        (*pQueue).psNodeQHead = (*(*pQueue).psNodeQHead).psNext;
        /* copy element from node for return */
        psElement = (*psNode).psElement;
        /* return node to free node list */
        (*psNode).psNext = (*pQueue).psFreeNodeList;
        (*pQueue).psFreeNodeList = psNode;
        /* update node counts */
        (*pQueue).iFreeNodes += 1;
        (*pQueue).iQueueNodes -= 1;
        if (*pQueue).iQueueNodes >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"queue_Dequeue: queue nodes < 0\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*pQueue).iQueueNodes >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"pqueue.c\x00" as *const u8 as *const libc::c_char,
                  251 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"queue_Dequeue\x00")).as_ptr(),
                  b"pQueue->iQueueNodes >= 0\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    return psElement;
}
/* **************************************************************************/
/*
 * queue_FindElement
 *
 * Find specific element in queue and return node pointer
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_FindElement(mut pQueue: *mut QUEUE,
                                           mut psElement: *mut libc::c_void)
 -> *mut QUEUE_NODE {
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    let mut psNodePrev: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    /* check input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_FindElement: queue pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              272 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"queue_FindElement\x00")).as_ptr(),
              b"PTRVALID(pQueue,sizeof(QUEUE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_FindElement: element pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              274 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"queue_FindElement\x00")).as_ptr(),
              b"PTRVALID(psElement,pQueue->iElementSize)\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* init pointers to head of queue */
    psNode = (*pQueue).psNodeQHead;
    psNodePrev = psNode;
    /* find node in queue */
    while !psNodePrev.is_null() && !psNode.is_null() {
        /* return TRUE if match found */
        if (*psNode).psElement == psElement { return psNode }
        /* update pointers */
        psNodePrev = psNode;
        psNode = (*psNode).psNext
    }
    return 0 as *mut QUEUE_NODE;
}
/* **************************************************************************/
/*
 * queue_GetHead
 *
 * Get element at head of queue
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_GetHead(mut pQueue: *mut QUEUE)
 -> *mut libc::c_void {
    /* set current node to head of queue */
    (*pQueue).psCurNode = (*pQueue).psNodeQHead;
    if !(*pQueue).psCurNode.is_null() {
        return (*(*pQueue).psCurNode).psElement
    } else { return 0 as *mut libc::c_void };
}
/* **************************************************************************/
/*
 * queue_GetNext
 *
 * Get next element in queue
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_GetNext(mut pQueue: *mut QUEUE)
 -> *mut libc::c_void {
    /* move to next node */
    if !(*pQueue).psCurNode.is_null() {
        (*pQueue).psCurNode = (*(*pQueue).psCurNode).psNext
    }
    /* return node element */
    if (*pQueue).psCurNode.is_null() {
        return 0 as *mut libc::c_void
    } else { return (*(*pQueue).psCurNode).psElement };
}
/* **************************************************************************/
/*
 * queue_RemoveCurrent
 *
 * Remove current node from queue
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_RemoveCurrent(mut pQueue: *mut QUEUE) -> BOOL {
    return queue_RemoveNode(pQueue, (*pQueue).psCurNode);
}
/* **************************************************************************/
/*
 * queue_RemoveNode
 *
 * Remove node from queue
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_RemoveNode(mut pQueue: *mut QUEUE,
                                          mut psNode: *mut QUEUE_NODE)
 -> BOOL {
    /* check input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_RemoveNode: queue pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              375 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"queue_RemoveNode\x00")).as_ptr(),
              b"PTRVALID(pQueue,sizeof(QUEUE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_RemoveNode: node pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              377 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"queue_RemoveNode\x00")).as_ptr(),
              b"PTRVALID(psNode,sizeof(QUEUE_NODE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* if node valid, remove from queue and return to free node list */
    if psNode.is_null() {
        return 0 as libc::c_int
    } else {
        /* remove node from queue */
        if (*psNode).psPrev.is_null() && (*psNode).psNext.is_null() {
            (*pQueue).psCurNode = 0 as *mut QUEUE_NODE;
            (*pQueue).psNodeQHead = 0 as *mut QUEUE_NODE
        } else {
            /* make previous node point to next */
            if !(*psNode).psPrev.is_null() {
                (*(*psNode).psPrev).psNext = (*psNode).psNext
            }
            /* make next node point to previous */
            if !(*psNode).psNext.is_null() {
                (*(*psNode).psNext).psPrev = (*psNode).psPrev
            }
            /* set current queue node if necessary */
            if (*pQueue).psCurNode == psNode {
                if !(*psNode).psPrev.is_null() {
                    (*pQueue).psCurNode = (*psNode).psPrev
                } else if !(*psNode).psNext.is_null() {
                    (*pQueue).psCurNode = (*psNode).psNext
                } else { (*pQueue).psCurNode = 0 as *mut QUEUE_NODE }
            }
            /* update queue pointer if neccessary */
            if (*pQueue).psNodeQHead == psNode {
                (*pQueue).psNodeQHead = (*psNode).psNext
            }
        }
        /* return node to free node list */
        (*psNode).psNext = (*pQueue).psFreeNodeList;
        (*pQueue).psFreeNodeList = psNode;
        /* update node counts */
        (*pQueue).iFreeNodes += 1;
        (*pQueue).iQueueNodes -= 1;
        if (*pQueue).iQueueNodes >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"queue_RemoveNode: queue nodes < 0\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*pQueue).iQueueNodes >= 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"pqueue.c\x00" as *const u8 as *const libc::c_char,
                  438 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"queue_RemoveNode\x00")).as_ptr(),
                  b"pQueue->iQueueNodes >= 0\x00" as *const u8 as
                      *const libc::c_char);
        };
        return 1 as libc::c_int
    };
}
/* **************************************************************************/
/*
 * queue_RemoveElement
 *
 * Remove specific element from queue
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn queue_RemoveElement(mut pQueue: *mut QUEUE,
                                             mut psElement: *mut libc::c_void)
 -> BOOL {
    let mut psNode: *mut QUEUE_NODE = 0 as *mut QUEUE_NODE;
    /* check input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_Dequeue: queue pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              459 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"queue_RemoveElement\x00")).as_ptr(),
              b"PTRVALID(pQueue,sizeof(QUEUE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"queue_RemoveElement: element pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"pqueue.c\x00" as *const u8 as *const libc::c_char,
              461 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"queue_RemoveElement\x00")).as_ptr(),
              b"PTRVALID(psElement,pQueue->iElementSize)\x00" as *const u8 as
                  *const libc::c_char);
    };
    psNode = queue_FindElement(pQueue, psElement);
    if !psNode.is_null() {
        queue_RemoveNode(pQueue, psNode);
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* **************************************************************************/
